package com.abyss.coral.data.database

import androidx.room.Database
import androidx.room.RoomDatabase
import com.abyss.coral.data.database.dao.AccountDao
import com.abyss.coral.data.database.entities.Account

@Database(entities=[Account::class], version=1, exportSchema=false)
abstract class CoralDatabase: RoomDatabase() {

    abstract fun accountDao(): AccountDao

}